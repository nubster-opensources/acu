//! A trivial deterministic substrate, used for tests and early development.

use crate::decision::Decision;
use crate::mind::MindState;
use crate::percept::Percept;
use crate::substrate::Substrate;

/// A trivial deterministic [`Substrate`] performing no input or output.
///
/// It keeps the cognitive core fully deterministic and testable without a network, and stands
/// in for richer substrates (a language model, a stateful neuron model) added later.
#[derive(Debug, Default)]
pub struct ReflexSubstrate;

impl Substrate for ReflexSubstrate {
    fn decide(&self, _state: &MindState, percept: &Percept) -> Decision {
        Decision {
            response: format!("ack: {}", percept.utterance),
        }
    }
}
