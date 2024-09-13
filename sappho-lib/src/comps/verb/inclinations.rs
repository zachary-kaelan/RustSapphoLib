use crate::consts::*;
use crate::BnumGroup;

pub const PERSONALITY_VERB_WEIGHTS: [[f32; BNUM_GROUP_SIZE]; BNUM_GROUP_SIZE] = [
    //  s_a      d_s      s_d      c_e
    [0.550, 0.100, 0.000, 0.100],   // bad_good
    [0.200, 0.750, 0.000, 0.250],   // faithless_honest
    [0.000, 0.000, 0.850, 0.000],   // timid_dominant
    [-0.250, -0.150, 0.150, 0.650], // ascetic_hedonistic
];

/*
Rationale:
  selfish_altruistic:
    bad_good (primary): generally we associate altruism with being good.
    faithless_honest: someone with more integrity is more likely to engage in altruism, even if just for mutual benefit.
    !ascetic_hedonistic: hedonism can often come at the expense of others (e.g., sadistic villains).
  deceptive_sincere:
    bad_good: sincerity isn't always good, but deception can often be used for nastiness.
    faithless_honest (primary)
    !ascetic_hedonistic: deception tends to be more fun.
  submissive_dominant:
    timid_dominant (primary)
    ascetic_hedonistic: being powerful tends to be more fun.
  cold_emotional:
    bad_good: being too cold is more likely to be seen as evil than being too emotional.
    faithless_honest: expressing oneself emotionally can be a component of honesty.
    ascetic_hedonistic (primary): hedonism is in pursuit of strong emotions.
 */

const ACCORDANCE_VERB_WEIGHTS: [[f32; BNUM_GROUP_SIZE]; BNUM_GROUP_SIZE] = [
    //  s_a      d_s      s_d      c_e
    [0.650, 0.000, 0.000, 0.100],   // misanthropy_altruism
    [0.350, 0.700, 0.000, 0.900],   // paranoia_gullibility
    [0.000, -0.300, -1.000, 0.000], // control_helplessness
    [0.000, 0.000, 0.000, 0.000],   // austerity_decadence
];

/*
Rationale:
  selfish_altruistic:
    misanthropy_altruism (primary): generally we associate altruism with being good.
    paranoia_gullibility: it's easier to stick your neck out for people if you think they won't stab you in the back.
  deceptive_sincere:
    paranoia_gullibility (primary): sincerity tends to be reciprocated with sincerity.
    !control_helplessness: if you feel in control, you're sincerer.
  submissive_dominant:
    !control_helplessness (primary)
  cold_emotional:
    misanthropy_altruism: only because misanthropy is often associated with apathy.
    paranoia_gullibility (primary): you're going to feel a lot more comfortable expressing yourself when you aren't paranoid.
 */

pub fn calculate_verb_weights(
    personality: BnumGroup,
    matrix: &[[f32; BNUM_GROUP_SIZE]; BNUM_GROUP_SIZE],
) -> BnumGroup {
    let mut verb_weights = [0.0f32; BNUM_GROUP_SIZE];

    for i in 0..BNUM_GROUP_SIZE {
        for j in 0..BNUM_GROUP_SIZE {
            verb_weights[i] += f32::from(personality.values[i]) * matrix[j][i];
        }
    }

    BnumGroup::from(verb_weights)
}
