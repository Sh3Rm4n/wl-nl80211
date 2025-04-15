// SPDX-License-Identifier: MIT

use crate::{Nl80211Handle, Nl80211SurveyGetRequest};

pub struct Nl80211SurveyHandle(Nl80211Handle);

impl Nl80211SurveyHandle {
    pub fn new(handle: Nl80211Handle) -> Self {
        Self(handle)
    }

    /// Retrieve the survey info
    /// (equivalent to `iw dev DEV survey dump`)
    pub fn dump(&mut self, if_index: u32) -> Nl80211SurveyGetRequest {
        Nl80211SurveyGetRequest::new(self.0.clone(), if_index)
    }
}
