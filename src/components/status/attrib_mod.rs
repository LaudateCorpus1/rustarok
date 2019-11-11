use crate::components::char::{CharAttributeModifier, CharAttributeModifierCollector, Percentage};
use crate::components::status::status::{
    Status, StatusNature, StatusUpdateParams, StatusUpdateResult,
};
use crate::ElapsedTime;

#[derive(Clone)]
pub struct ArmorModifierStatus {
    pub started: ElapsedTime,
    pub until: ElapsedTime,
    pub modifier: Percentage,
}

impl ArmorModifierStatus {
    pub fn new(now: ElapsedTime, modifier: Percentage) -> ArmorModifierStatus {
        ArmorModifierStatus {
            started: now,
            until: now.add_seconds(10.0),
            modifier,
        }
    }
}

impl Status for ArmorModifierStatus {
    fn dupl(&self) -> Box<dyn Status + Send> {
        Box::new(self.clone())
    }

    fn calc_attribs(&self, modifiers: &mut CharAttributeModifierCollector) {
        modifiers.change_armor(
            CharAttributeModifier::AddPercentage(self.modifier),
            self.started,
            self.until,
        );
    }

    fn update(&mut self, params: StatusUpdateParams) -> StatusUpdateResult {
        if self.until.has_already_passed(params.sys_vars.time) {
            StatusUpdateResult::RemoveIt
        } else {
            StatusUpdateResult::KeepIt
        }
    }

    fn typ(&self) -> StatusNature {
        StatusNature::Supportive // depends
    }
}

#[derive(Clone)]
pub struct WalkingSpeedModifierStatus {
    pub started: ElapsedTime,
    pub until: ElapsedTime,
    pub modifier: Percentage,
}

impl WalkingSpeedModifierStatus {
    pub fn new(
        now: ElapsedTime,
        modifier: Percentage,
        duration: f32,
    ) -> WalkingSpeedModifierStatus {
        WalkingSpeedModifierStatus {
            started: now,
            until: now.add_seconds(duration),
            modifier,
        }
    }
}

impl Status for WalkingSpeedModifierStatus {
    fn dupl(&self) -> Box<dyn Status + Send> {
        Box::new(self.clone())
    }

    fn calc_attribs(&self, modifiers: &mut CharAttributeModifierCollector) {
        modifiers.change_walking_speed(
            CharAttributeModifier::AddPercentage(self.modifier),
            self.started,
            self.until,
        );
    }

    fn update(&mut self, params: StatusUpdateParams) -> StatusUpdateResult {
        if self.until.has_already_passed(params.sys_vars.time) {
            StatusUpdateResult::RemoveIt
        } else {
            StatusUpdateResult::KeepIt
        }
    }

    fn typ(&self) -> StatusNature {
        StatusNature::Supportive // depends
    }
}
