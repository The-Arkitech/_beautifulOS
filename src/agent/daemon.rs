#![no_std]

use log::{info, warn};
use crate::system_calls::syscall_execute;
// Note: In a complete kernel, this maps to our bare-metal inference engine
use crate::llm_engine::{Model, LlmResponse}; 

pub struct AgenticDaemon {
    is_active: bool,
    model: &'static Model,
}

impl AgenticDaemon {
    pub fn new(loaded_model: &'static Model) -> Self {
        Self {
            is_active: true,
            model: loaded_model,
        }
    }

    /// Ingests natural language and outputs bare-metal execution.
    pub fn process_directive(&self, human_input: &str) {
        if !self.is_active {
            return;
        }

        info!("Daemon ingesting directive: {}", human_input);

        // 1. INFERENCE: The quantized model translates text to actionable JSON/Commands.
        // We instruct the model via system prompt to ONLY output system call parameters.
        let prompt = format!(
            "System Prompt: You are the core interface for _beautifulOS. 
            Translate the user request into a strict system command ID and target.
            User Request: {}", 
            human_input
        );

        let response: LlmResponse = self.model.infer(&prompt);

        // 2. EXECUTION: The daemon parses the LLM's structured output.
        match response.command_type {
            "QUARANTINE_PID" => {
                warn!("Daemon authorized resource mutation. Target PID: {}", response.target);
                // 3. SEVERANCE: Pass the translated command down to Ring 0.
                unsafe { syscall_execute(1, response.target); }
            },
            "ALLOCATE_RAM" => {
                info!("Daemon authorized memory expansion for subsystem.");
                unsafe { syscall_execute(2, response.target); }
            },
            _ => {
                warn!("Daemon rejected directive. Ambiguous translation.");
            }
        }
    }
}