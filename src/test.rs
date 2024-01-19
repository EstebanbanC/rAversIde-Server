
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{analyze::{CodeAnalysisRequest, analyze}, chatbot::{handle_chatbot, QuestionRequest, WsChannel}, rename::{RenameRequest, RenameItem, rename_function}};
    use crate::analyze::FormattedChatResponse;
    use crate::rename::RenameResponse;

    use rocket::{serde::json::Json, State};
    
    
    #[tokio::test]
    async fn test_analyze_success() {
        let mut code_c_map = HashMap::new();
        code_c_map.insert("main".to_string(), r#"
        undefined8 main(void)
        {
        undefined8 local_48;
        undefined8 local_40;
        undefined8 local_38;
        undefined8 local_30;
        undefined8 local_28;
        undefined8 local_20;
        undefined2 local_18;
        uint local_c;
        
        setbuf(stdout,(char *)0x0);
        local_c = 0xaabdcdee;
        local_48 = 0;
        local_40 = 0;
        local_38 = 0;
        local_30 = 0;
        local_28 = 0;
        local_20 = 0;
        local_18 = 0;
        printf("Enter your favorite number: ");
        fgets((char *)&local_48,0x50,stdin);
        if (local_c == 0x45454545) {
            afficherDrapeau();
        }
        else {
            printf("Too bad! The flag is 0x%x\n",(ulong)local_c);
        }
        return 0;
        }"#.to_string());

        let code_analysis_request = CodeAnalysisRequest {
            action: "Analyse".to_string(),
            r#type: "vulnérabilité".to_string(),
            code_asm: {
                let mut code_asm_map = HashMap::new();
                code_asm_map.insert("main".to_string(), vec![
                    vec!["00101209".to_string(), "ENDBR64".to_string()],
                    vec!["0010120d".to_string(), "PUSH RBP".to_string()],
                    vec!["0010120e".to_string(), "MOV RBP,RSP".to_string()],
                    vec!["00101211".to_string(), "PUSH RBX".to_string()],
                    vec!["00101212".to_string(), "SUB RSP,0x38".to_string()],
                    vec!["00101216".to_string(), "MOV byte ptr [RBP + -0x11],0x0".to_string()],
                    vec!["0010121a".to_string(), "MOV qword ptr [RBP + -0x40],0x0".to_string()],
                    vec!["00101222".to_string(), "MOV qword ptr [RBP + -0x38],0x0".to_string()],
                    vec!["0010122a".to_string(), "MOV qword ptr [RBP + -0x30],0x0".to_string()],
                    vec!["00101232".to_string(), "MOV qword ptr [RBP + -0x28],0x0".to_string()],
                    vec!["0010123a".to_string(), "LEA RAX,[0x102004]".to_string()],
                    vec!["00101241".to_string(), "MOV RDI,RAX".to_string()],
                    vec!["00101244".to_string(), "MOV EAX,0x0".to_string()],
                    vec!["00101249".to_string(), "CALL 0x001010d0".to_string()],
                    vec!["0010124e".to_string(), "LEA RAX,[RBP + -0x40]".to_string()],
                    vec!["00101252".to_string(), "MOV RDI,RAX".to_string()],
                    vec!["00101255".to_string(), "MOV EAX,0x0".to_string()],
                    vec!["0010125a".to_string(), "CALL 0x00101100".to_string()],
                    vec!["0010125f".to_string(), "LEA RAX,[RBP + -0x40]".to_string()],
                    vec!["00101263".to_string(), "LEA RDX,[0x102022]".to_string()],
                    vec!["0010126a".to_string(), "MOV RSI,RDX".to_string()],
                    vec!["0010126d".to_string(), "MOV RDI,RAX".to_string()],
                    vec!["00101270".to_string(), "CALL 0x001010f0".to_string()],
                    vec!["00101275".to_string(), "TEST EAX,EAX".to_string()],
                    vec!["00101277".to_string(), "JNZ 0x001012b2".to_string()],
                    vec!["00101279".to_string(), "CMP byte ptr [RBP + -0x11],0x0".to_string()],
                    vec!["0010127d".to_string(), "JZ 0x001012b2".to_string()],
                    vec!["0010127f".to_string(), "LEA RAX,[0x102028]".to_string()],
                    vec!["00101286".to_string(), "MOV RDI,RAX".to_string()],
                    vec!["00101289".to_string(), "CALL 0x001010b0".to_string()],
                    vec!["0010128e".to_string(), "CALL 0x001010e0".to_string()],
                    vec!["00101293".to_string(), "MOV EBX,EAX".to_string()],
                    vec!["00101295".to_string(), "CALL 0x001010e0".to_string()],
                    vec!["0010129a".to_string(), "MOV ESI,EBX".to_string()],
                    vec!["0010129c".to_string(), "MOV EDI,EAX".to_string()],
                    vec!["0010129e".to_string(), "CALL 0x00101110".to_string()],
                    vec!["001012a3".to_string(), "LEA RAX,[0x102038]".to_string()],
                    vec!["001012aa".to_string(), "MOV RDI,RAX".to_string()],
                    vec!["001012ad".to_string(), "CALL 0x001010c0".to_string()],
                    vec!["001012b2".to_string(), "NOP".to_string()],
                    vec!["001012b3".to_string(), "MOV RBX,qword ptr [RBP + -0x8]".to_string()],
                    vec!["001012b7".to_string(), "LEAVE".to_string()],
                    vec!["001012b8".to_string(), "RET".to_string()],
                ]);
                code_asm_map
            },
            code_c: code_c_map,
        };
    
        // Appel de la fonction `analyze` avec les données mockées
        let response = analyze(Json(code_analysis_request)).await;
    
        // Vérifier que la réponse est sous le bon format
        let response_data: Result<FormattedChatResponse, serde_json::Error> = serde_json::from_str(&response);
        assert!(response_data.is_ok(), "La désérialisation a échoué, le format de réponse n'est pas correct.");
    }
    

    #[tokio::test]
    async fn test_handle_chatbot_success() {
        let mut code_c_map = HashMap::new();
        code_c_map.insert("main".to_string(), r#"
        undefined8 main(void)
        {
        undefined8 local_48;
        undefined8 local_40;
        undefined8 local_38;
        undefined8 local_30;
        undefined8 local_28;
        undefined8 local_20;
        undefined2 local_18;
        uint local_c;
        
        setbuf(stdout,(char *)0x0);
        local_c = 0xaabdcdee;
        local_48 = 0;
        local_40 = 0;
        local_38 = 0;
        local_30 = 0;
        local_28 = 0;
        local_20 = 0;
        local_18 = 0;
        printf("Enter your favorite number: ");
        fgets((char *)&local_48,0x50,stdin);
        if (local_c == 0x45454545) {
            afficherDrapeau();
        }
        else {
            printf("Too bad! The flag is 0x%x\n",(ulong)local_c);
        }
        return 0;
        }"#.to_string());


        let question_request = QuestionRequest {
            action: "action_exemple".to_string(),
            question: "Quelle est la fonction de cette portion de code?".to_string(),
            code_c: code_c_map,
        };

        let ws_channel = WsChannel::new(tokio::sync::broadcast::channel::<String>(32).0);
        let ws_channel_state = &State::from(&ws_channel);
        // Appel de handle_chatbot avec les données mockées
        let response = handle_chatbot(Json(question_request), ws_channel_state).await;
        assert!(!response.is_empty(), "La réponse ne devrait pas être vide.");
    }

    #[tokio::test]
    async fn test_rename_function_success() {

        let mut code_c_map = HashMap::new();
        code_c_map.insert("main".to_string(), r#"
        undefined8 main(void)
        {
        undefined8 local_48;
        undefined8 local_40;
        undefined8 local_38;
        undefined8 local_30;
        undefined8 local_28;
        undefined8 local_20;
        undefined2 local_18;
        uint local_c;
        
        setbuf(stdout,(char *)0x0);
        local_c = 0xaabdcdee;
        local_48 = 0;
        local_40 = 0;
        local_38 = 0;
        local_30 = 0;
        local_28 = 0;
        local_20 = 0;
        local_18 = 0;
        printf("Enter your favorite number: ");
        fgets((char *)&local_48,0x50,stdin);
        if (local_c == 0x45454545) {
            afficherDrapeau();
        }
        else {
            printf("Too bad! The flag is 0x%x\n",(ulong)local_c);
        }
        return 0;
        }"#.to_string());


        let rename_request = RenameRequest {
            items: vec![
                RenameItem {
                    item_type: "fonction".to_string(),
                    old_name: "main".to_string(),
                },
            ],
            code_c: code_c_map,
        };

        // Appel de la fonction `rename_function` avec les données mockées
        let response = rename_function(Json(rename_request)).await;

        // Vérifier que la réponse est sous le bon format
        let response_data: Result<RenameResponse, serde_json::Error> = serde_json::from_str(&response);
        assert!(response_data.is_ok(), "La désérialisation a échoué, le format de réponse n'est pas correct.");
        
    }

    #[tokio::test]
    async fn test_rename_variable_success() {

        let mut code_c_map = HashMap::new();
        code_c_map.insert("main".to_string(), r#"
        undefined8 main(void)
        {
        undefined8 local_48;
        undefined8 local_40;
        undefined8 local_38;
        undefined8 local_30;
        undefined8 local_28;
        undefined8 local_20;
        undefined2 local_18;
        uint local_c;
        
        setbuf(stdout,(char *)0x0);
        local_c = 0xaabdcdee;
        local_48 = 0;
        local_40 = 0;
        local_38 = 0;
        local_30 = 0;
        local_28 = 0;
        local_20 = 0;
        local_18 = 0;
        printf("Enter your favorite number: ");
        fgets((char *)&local_48,0x50,stdin);
        if (local_c == 0x45454545) {
            afficherDrapeau();
        }
        else {
            printf("Too bad! The flag is 0x%x\n",(ulong)local_c);
        }
        return 0;
        }"#.to_string());


        let rename_request = RenameRequest {
            items: vec![
                RenameItem {
                    item_type: "variable".to_string(),
                    old_name: "main".to_string(),
                },
            ],
            code_c: code_c_map,
        };

        // Appel de la fonction `rename_function` avec les données mockées
        let response = rename_function(Json(rename_request)).await;

        // Vérifier que la réponse est sous le bon format
        let response_data: Result<RenameResponse, serde_json::Error> = serde_json::from_str(&response);
        assert!(response_data.is_ok(), "La désérialisation a échoué, le format de réponse n'est pas correct.");
    }
}

