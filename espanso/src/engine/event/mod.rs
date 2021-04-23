/*
 * This file is part of espanso.
 *
 * Copyright (C) 2019-2021 Federico Terzi
 *
 * espanso is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * espanso is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with espanso.  If not, see <https://www.gnu.org/licenses/>.
 */

pub mod input;
pub mod effect;
pub mod internal;

#[derive(Debug, Clone)]
pub enum Event {
  NOOP,
  ProcessingError(String),
  
  // Inputs
  Keyboard(input::KeyboardEvent),

  // Internal
  MatchesDetected(internal::MatchesDetectedEvent),
  MatchSelected(internal::MatchSelectedEvent),
  CauseCompensatedMatch(internal::CauseCompensatedMatchEvent),

  RenderingRequested(internal::RenderingRequestedEvent),
  Rendered(internal::RenderedEvent),
  MatchInjected,

  // Effects
  TriggerCompensation(effect::TriggerCompensationEvent),
  CursorHintCompensation(effect::CursorHintCompensationEvent),

  KeySequenceInject(effect::KeySequenceInjectRequest),
  TextInject(effect::TextInjectRequest),
}