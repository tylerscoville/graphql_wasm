// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use apollo_parser::{cst, Parser};
use log::{info, warn};
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(HttpBodyRoot) });
}}

struct HttpBodyRoot;

impl Context for HttpBodyRoot {}

impl RootContext for HttpBodyRoot {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(HttpBody { context_id }))
    }
}

struct HttpBody {
    context_id: u32,
}

impl Context for HttpBody {}

impl HttpContext for HttpBody {
    // fn on_http_request_headers(&mut self, _num_headers: usize, _end_of_stream: bool) -> Action {

    //     for (name, value) in &self.get_http_request_headers() {
    //         info!("ALSKJF: #{} -> {}: {}", self.context_id, name, value);
    //     }

    //     self.set_http_request_header(":authority", Some("echo_service:5678"));
    //     self.set_http_request_header(":path", Some("/"));


    //     Action::Continue
    // }


    fn on_http_request_body(&mut self, body_size: usize, end_of_stream: bool) -> Action {
        info!("Original message body ({body_size} bytes).\n");

        if !end_of_stream {
            return proxy_wasm::types::Action::Pause;
        }

        let body = self.get_http_request_body(0, body_size).unwrap_or_default();
        let input = String::from_utf8(body).unwrap_or_default();

        let parser = Parser::new(&input);
        let cst = parser.parse();
        if cst.errors().len() > 0 {
            for error in cst.errors() {
                warn!("FOUND ERROR: {}", error)
            }
        }
        let doc = cst.document();

        info!("NUMBER OF DEFINITIONS: {}", doc.definitions().count());

        for def in doc.definitions() {
            info!("DEFINITION TYPE: {}", def.kind());
            if let cst::Definition::OperationDefinition(operation_def) = def {
                let operation_name = match operation_def.name() {
                    // The division was valid
                    Some(n) => n.text().to_string(),
                    // The division was invalid
                    None => "UNKNOWN".to_string(),
                };
                let mut found_fields = Vec::new();
                if let Some(selection_set) = operation_def.selection_set() {
                    for selection in selection_set.selections() {
                        match selection {
                            cst::Selection::Field(field) => {
                                found_fields.push(field.name().unwrap().text().to_string());
                            }
                            _ => {}
                        }
                    }
                }

                let response: String = format!(
                    "For {} we found this selection: {:?}",
                    operation_name.to_string(),
                    found_fields
                );

                info!("doing this: {}", response);

                for (name, value) in self.get_http_request_headers() {
                    info!("{}: {}", name, value);
                }

                self.send_http_response(200, Vec::default(), Some(response.as_bytes()));
            }
        }
        Action::Continue
    }
}
