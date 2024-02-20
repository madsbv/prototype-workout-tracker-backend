use crate::em_exercise_data::EmExerciseSpecification;
use crate::Exercise;

pub fn default_exercise_list() -> Vec<Exercise> {
    let mut em_rdr = csv::Reader::from_reader(EM_EXERCISE_SPECS.as_bytes());
    em_rdr
        .deserialize::<EmExerciseSpecification>()
        .map(|em| em.expect("default list exercises parse correctly").into())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ensure that the
    #[test]
    fn test_default_exercise_list() {
        let defaults = default_exercise_list();
        assert!(!defaults.is_empty());
    }
}

const EM_EXERCISE_SPECS: &str = r#"
,exercise,two sided,exercise type,equipment,equipment type,muscle groups,routine type,Unnamed: 7
0,bow pose,no,stretch,no,none,"lower back, quadriceps, front deltoid, abdominals",bridge,
1,bridge,no,stretch,no,none,"lower back, quadriceps, front deltoid, abdominals, wrists, lats, side deltoid",bridge,goal
2,broom stretch,yes,stretch,yes,stick,"pectorals, front deltoid",upper back,
3,camel pose,no,stretch,no,none,"lower back, quadriceps, front deltoid, abdominals",bridge,
4,shoulder dislocations,no,stretch,yes,stick or strap,"front deltoid, rear deltoid, side deltoid, triceps",bridge,
5,superman,no,bodyweight,no,none,"lower back, front deltoid, rear deltoid, rotator cuffs, side deltoid, abdominals, obliques",bridge,
6,table top,no,stretch,no,none,"front deltoid, biceps",bridge,
7,upward facing dog,no,stretch,no,none,lower back,bridge,
8,dip,no,bodyweight,yes,dip stand,"pectorals, front deltoid, triceps","dip, triceps",goal
9,"dip, bench",no,bodyweight,yes,bench,"triceps, pectorals, front deltoid","dip, triceps",
10,"lever, front",no,bodyweight,yes,pullup bar,"lats, rear deltoid, trapezius, erector spinae, abdominals, obliques, glutes, forearm flexors, forearm extensors",front lever,
11,skin the cat,no,bodyweight,yes,pullup bar,"lats, pectorals, biceps, front deltoid, rear deltoid, side deltoid, rhomboids",front lever,goal
12,"pushup, horizontal",no,bodyweight,yes,rings or step,"pectorals, front deltoid, triceps, abdominals",handstand pushup,
13,bicycles,no,bodyweight,no,none,"abdominals, obliques, quadriceps",hiit,
14,box jumps,no,cardio,yes,box,"quadriceps, glutes, adductors, lower back, calves, abdominals",hiit,
15,burpees,no,cardio,no,none,"pectorals, front deltoid, triceps, abdominals, quadriceps, glutes, adductors, lower back, calves",hiit,
16,high knees,no,cardio,no,none,"quadriceps, glutes, adductors, lower back, calves, abdominals",hiit,
17,kettlebell swing,no,lift,yes,kettlebell,"glutes, lower back, hamstrings, adductors, trapezius, forearm flexors",hiit,
18,lemon squeezers,no,bodyweight,no,none,"abdominals, obliques, quadriceps",hiit,
19,"lunges, jumping",no,bodyweight,no,none,"quadriceps, glutes, adductors, abdominals",hiit,
20,mountain climbers,no,cardio,no,none,"abdominals, obliques",hiit,
21,squat,no,bodyweight,no,none,"quadriceps, glutes, adductors, calves",hiit,
22,"squat, goblet",no,lift,yes,kettlebell,"quadriceps, glutes, adductors, lower back, calves",hiit,
23,tuck jumps,no,cardio,no,none,"quadriceps, glutes, adductors, lower back, calves, abdominals",hiit,
24,clean,no,lift,yes,barbell,"glutes, lower back, adductors, quadriceps, hamstrings, trapezius, forearm flexors, calves",no,
25,clean and jerk,no,lift,yes,barbell,"glutes, lower back, adductors, quadriceps, front deltoid, hamstrings, trapezius, forearm flexors, triceps, side deltoid, calves",no,
26,hang clean,no,lift,yes,barbell,"glutes, lower back, adductors, quadriceps, hamstrings, trapezius, forearm flexors, calves",no,
27,hang clean power snatch,no,lift,yes,barbell,"glutes, lower back, adductors, hamstrings, trapezius, forearm flexors",no,
28,power clean,no,lift,yes,barbell,"glutes, lower back, quadriceps, hamstrings, adductors, trapezius, forearm flexors",no,
29,power jerk,no,lift,yes,"barbell, rack","front deltoid, quadriceps, glutes, adductors, lower back, trapezius, triceps, side deltoid",no,
30,split jerk,no,lift,yes,"barbell, rack","front deltoid, quadriceps, glutes, adductors, lower back, trapezius, triceps, side deltoid",no,
31,squat jerk,no,lift,yes,"barbell, rack","glutes, lower back, adductors, quadriceps, front deltoid, trapezius, triceps, side deltoid, calves",no,
32,"squat, pistol",yes,bodyweight,no,none,"quadriceps, abdominals, obliques, glutes, hamstrings, ankles",pistol squat,goal
33,planche,no,bodyweight,no,none,"pectorals, front deltoid, rotator cuffs, biceps, triceps, wrists, rhomboids, trapezius, abdominals, obliques, erector spinae, quadriceps, calves, hamstrings",planche,
34,"planche, spilt",no,bodyweight,no,none,"front deltoid, abdominals, pectorals, glutes, wrists, rhomboids, trapezius, quadriceps, calves, triceps, biceps, rotator cuffs",planche,
35,"planche, tuck",no,bodyweight,no,none,"front deltoid, abdominals, pectorals, glutes, wrists, rhomboids, trapezius, quadriceps, calves, triceps, biceps, rotator cuffs",planche,
36,ab wheel rollout,no,bodyweight,yes,wheel,"abdominals, obliques",press handstand,
37,frog stretch,no,stretch,no,none,"adductors, abductors",press handstand,
38,handstand,no,bodyweight,no,none,"lats, trapezius, erector spinae, rear deltoid, front deltoid, glutes, abdominals, triceps, forearm extensors, forearm flexors, obliques, quadriceps, wrists",press handstand,goal
39,handstand pushup,no,bodyweight,no,none or wall,"lats, trapezius, erector spinae, rear deltoid, front deltoid, glutes, abdominals, triceps, forearm extensors, forearm flexors, obliques, quadriceps, wrists",press handstand,goal
40,"handstand, forearm",no,bodyweight,no,none,"lats, trapezius, erector spinae, rear deltoid, front deltoid, glutes, abdominals, triceps, forearm extensors, forearm flexors, obliques, quadriceps",press handstand,goal
41,"handstand, one arm",yes,bodyweight,no,none or wall,"lats, trapezius, erector spinae, rear deltoid, front deltoid, glutes, abdominals, triceps, forearm extensors, forearm flexors, obliques, quadriceps, wrists",press handstand,
42,"handstand, wall, back",no,bodyweight,yes,wall,"lats, trapezius, erector spinae, rear deltoid, front deltoid, glutes, abdominals, triceps, forearm extensors, forearm flexors, obliques, quadriceps, wrists",press handstand,
43,"handstand, wall, front",no,bodyweight,yes,wall,"lats, trapezius, erector spinae, rear deltoid, front deltoid, glutes, abdominals, triceps, forearm extensors, forearm flexors, obliques, quadriceps, wrists",press handstand,
44,leg lift,no,bodyweight,yes,pullup bar,"abdominals, obliques",press handstand,goal
45,"leg lifts, prone",no,bodyweight,yes,none,"abdominals, obliques",press handstand,
46,"rainbow, hanging",yes,bodyweight,yes,pullup bar,"abdominals, obliques",press handstand,
47,"rainbow, hanging, bent knees",yes,bodyweight,yes,pullup bar,"abdominals, obliques",press handstand,
48,v up,no,bodyweight,no,none,"abdominals, obliques, quadriceps",press handstand,goal
49,"windshield wiper, prone",yes,bodyweight,no,none,"abdominals, obliques",press handstand,
50,"windshield wiper, prone, bent knees",yes,bodyweight,no,none,"abdominals, obliques",press handstand,
51,"bent arm hang, pronated",no,bodyweight,yes,pullup bar,"forearm flexors, trapezius, triceps, biceps, rhomboids, front deltoid, rear deltoid, abdominals, obliques",pullup,
52,"bent arm hang, supinated",no,bodyweight,yes,pullup bar,"forearm flexors, trapezius, triceps, biceps, rhomboids, front deltoid, rear deltoid, abdominals, obliques, pectorals",pullup,
53,chinup,no,bodyweight,yes,pullup bar,"lats, biceps, rear deltoid, forearm flexors, rotator cuffs",pullup,
54,"dead hang, pronated",no,bodyweight,yes,pullup bar,"forearm flexors, trapezius, triceps, biceps, rhomboids, front deltoid, rear deltoid, abdominals, obliques",pullup,
55,"dead hang, one arm, pronated",yes,bodyweight,yes,pullup bar,"forearm flexors, trapezius, triceps, biceps, rhomboids, front deltoid, rear deltoid, abdominals, obliques",pullup,
56,lat pulldown,no,lift,yes,cable machine,"lats, biceps, rear deltoid, forearm flexors, rotator cuffs",pullup,
57,"lat pulldown, straight arm",no,lift,yes,cable machine,"lats, rear deltoid, rotator cuffs",pullup,
58,lock off,yes,bodyweight,yes,pullup bar,"forearm flexors, trapezius, triceps, biceps, rhomboids, front deltoid, rear deltoid, abdominals, obliques",pullup,
59,pullup,no,bodyweight,yes,pullup bar,"lats, biceps, rear deltoid, forearm flexors, rotator cuffs",pullup,goal
60,"row, dumbbell",yes,lift,yes,"dumbbell, bench","rear deltoid, trapezius, rotator cuffs, biceps, forearm flexors",pullup,
61,"row, inverted",no,bodyweight,yes,rack or smith machine,"lats, trapezius, rear deltoid, biceps, forearm flexors, rotator cuffs",pullup,
62,"hip abduction, machine",no,lift,yes,hip abduction machine,"abductors, glutes",sits,
63,"hip abduction, resistance band",no,lift,yes,resistance band,"abductors, glutes",sits,
64,split reach-through,no,bodyweight,no,none,"abdominals, obliques, adductors, abductors","sits, press handstand, split",
65,split toe circles,yes,bodyweight,no,none,"abdominals, obliques, adductors, abductors","sits, press handstand, split",
66,fire hydrants,yes,bodyweight,no,none,glutes,split,
67,low lunge,yes,stretch,no,none,"adductors, quadriceps",split,
68,"pigeon pose, prone",yes,stretch,no,none,"abductors, glutes, hamstrings",split,
69,"pigeon pose, upright",yes,stretch,no,none,"abductors, glutes, hamstrings, lower back",split,
70,runners lunge,yes,stretch,no,none,"hamstrings, calves, glutes",split,
71,"split, middle",no,stretch,no,none,"adductors, glutes, abductors, quadriceps, hamstrings, calves, lower back",split,goal
72,"split, middle, pancake",no,stretch,no,none,"adductors, glutes, abductors, quadriceps, hamstrings, calves, trapezius, lower back, rear deltoid, side deltoid",split,goal
73,"split, middle, upright",no,stretch,no,none,"adductors, abductors, quadriceps, hamstrings",split,goal
74,"split, side",yes,stretch,no,none,"hamstrings, glutes, adductors, abductors, adductors, ",split,
75,"split, middle, side stretch",yes,stretch,no,none,"adductors, glutes, abductors, quadriceps, hamstrings, calves, lower back, obliques, lats, rear deltoid","split, upper back",
76,"split, middle, side stretch, overhead",yes,stretch,no,none,"adductors, glutes, abductors, quadriceps, hamstrings, calves, lower back, obliques, rear deltoid, triceps, rhomboids","split, upper back",
77,"triceps extension, cable, overhead",no,lift,yes,cable machine,triceps,triceps,
78,"triceps extension, dumbbell, lying",no,lift,yes,"dumbbell, bench",triceps,triceps,
79,"triceps extension, dumbbell, standing",no,lift,yes,dumbbell,triceps,triceps,
80,triceps pushdown,no,lift,yes,cable machine,triceps,triceps,
81,arm circles,no,stretch,no,none,"front deltoid, rear deltoid, side deltoid, triceps, biceps",upper back,
82,backward arms triangle,no,stretch,no,none,"front deltoid, lats, biceps",upper back,
83,crossed arms stretch,yes,stretch,no,none,"triceps, rear deltoid",upper back,
84,diver’s stretch,no,stretch,no,none,"lats, front deltoid, side deltoid, forearm flexors, rear deltoid",upper back,
85,eagle arms,yes,stretch,no,none,"triceps, biceps, side deltoid, rear deltoid, trapezius, rhomboids",upper back,
86,"external shoulder rotation, dumbbell, horizontal",yes,lift,yes,dumbbell,rotator cuffs,upper back,
87,"external shoulder rotation, dumbbell, lying",yes,lift,yes,dumbbell,rotator cuffs,upper back,
88,"external shoulder rotation, resistance band",yes,lift,yes,"resistance band, rack",rotator cuffs,upper back,
89,"internal shoulder rotation, dumbbel, lying",yes,lift,yes,dumbbell,rotator cuffs,upper back,
90,"internal shoulder rotation, dumbbell, horizontal",yes,lift,yes,dumbbell,rotator cuffs,upper back,
91,"internal shoulder rotation, resistance band",yes,lift,yes,"resistance band, rack",rotator cuffs,upper back,
92,monkey row,no,lift,yes,dumbbell,"side deltoid, front deltoid, trapezius, biceps",upper back,
93,overhead bent arm stretch,yes,stretch,no,none,"triceps, lats",upper back,
94,resistance band pull apart,no,lift,yes,resistance band,"rear deltoid, rotator cuffs, trapezius",upper back,
95,"reverse flyes, dumbbell",no,lift,yes,dumbbell,"rear deltoid, rotator cuffs, trapezius",upper back,
96,"reverse flyes, machine",no,lift,yes,reverse fly machine,"rear deltoid, rotator cuffs, trapezius",upper back,
97,"seated forward fold, one leg, toes flexed",yes,stretch,no,none,"glutes, hamstrings, lower back, trapezius, calves",upper back,
98,"seated forward fold, one leg, toes pointed",yes,stretch,no,none,"glutes, hamstrings, lower back, trapezius",upper back,
99,"seated forward fold, toes flexed",no,stretch,no,none,"glutes, hamstrings, lower back, trapezius, calves",upper back,
100,"seated forward fold, toes pointed",no,stretch,no,none,"glutes, hamstrings, lower back, trapezius",upper back,
101,wall clock,yes,stretch,yes,wall,"front deltoid, rear deltoid, side deltoid, triceps, biceps",upper back,
102,"wrist curl, dumbbell",yes,lift,yes,dumbbell,forearm flexors,wrist,
103,"wrist curls, barbell",no,lift,yes,light barbell,forearm flexors,wrist,
104,"wrist extension, barbell",no,lift,yes,barbell,forearm extensors,wrist,
105,"wrist extension, dumbbell",yes,lift,yes,dumbbell,forearm extensors,wrist,
106,wrist stretches,yes,stretch,no,none,wrists,wrist,
107,back extension,no,lift,yes,"back extension bench, weight plate","glutes, lower back, hamstrings",,
108,banded side kick,yes,lift,yes,resistance band,"abductors, glutes",,
109,"bench press, barbell",no,lift,yes,"barbell, rack, bench","pectorals, front deltoid, triceps",,goal
110,"bench press, barbell, close-grip",no,lift,yes,"barbell, bench","triceps, pectorals, front deltoid",,
111,"bench press, barbell, decline",no,lift,yes,"barbell, bench","pectorals, triceps, front deltoid",,
112,"bench press, barbell, incline",no,lift,yes,"incline bench, barbell, rack","pectorals, front deltoid, triceps",,
113,"bench press, dumbbell",no,lift,yes,"dumbbell, bench","pectorals, front deltoid, triceps",,
114,"bench press, dumbbell, decline",no,lift,yes,"dumbbell, bench","pectorals, front deltoid, triceps",,
115,"bench press, dumbbell, floor",no,lift,yes,dumbbell,"pectorals, front deltoid, triceps",,
116,"bench press, dumbbell, incline",no,lift,yes,"incline bench, dumbbell","pectorals, front deltoid, triceps",,
117,"bench press, smith machine",no,lift,yes,"smith machine, bench","pectorals, front deltoid, triceps",,
118,"bench press, smith machine, incline",no,lift,yes,"smith machine, bench","pectorals, front deltoid, triceps",,
119,"boat pose, bent legs",no,bodyweight,no,none,"abdominals, obliques, quadriceps, hamstrings",,
120,"boat pose, straight legs",no,bodyweight,no,none,"abdominals, obliques, quadriceps, hamstrings",,
121,chair pose,no,bodyweight,no,none,"quadriceps, calves, glutes",,
122,"chest fly, cable",no,lift,yes,cable machine,"pectorals, front deltoid",,
123,"chest fly, dumbbell",no,lift,yes,"dumbbell, bench","pectorals, front deltoid",,
124,"chest fly, machine",no,lift,yes,chest fly machine,"pectorals, front deltoid",,
125,"chest fly, resistance band",yes,lift,yes,"resistance band, rack","pectorals, front deltoid",,
126,"chest press, cable",no,lift,yes,cable machine,"pectorals, front deltoid, triceps",,
127,"chest press, machine",no,lift,yes,chest press machine,"pectorals, front deltoid, triceps",,
128,clamshells,yes,bodyweight,no,none,"abductors, glutes",,
129,concentration curl,yes,lift,yes,"dumbbell, bench","biceps, forearm flexors",,
130,crocodile pose,no,bodyweight,no,none,"abdominals, front deltoid, biceps, forearm flexors, pectorals",,
131,crunch,no,bodyweight,no,none,"abdominals, obliques",,
132,"crunch, cable",no,lift,yes,cable machine,"abdominals, obliques",,
133,"crunch, decline",no,lift,yes,decline bench,"abdominals, obliques",,
134,"crunch, machine",no,lift,yes,crunch machine,"abdominals, obliques",,
135,"crunch, oblique",yes,bodyweight,no,none,"abdominals, obliques",,
136,"curl, barbell",no,lift,yes,barbell or light barbell,"biceps, forearm flexors",,
137,"curl, barbell, preacher",no,lift,yes,"barbell or light barbell, preacher bench","biceps, forearm flexors",,
138,"curl, bodyweight",no,bodyweight,yes,rings or trx,"biceps, lats, forearm flexors",,
139,"curl, cable",no,lift,yes,cable machine,"biceps, forearm flexors",,
140,"curl, dumbbell",no,lift,yes,dumbbell,"biceps, forearm flexors",,
141,"curl, dumbbell, hammer",no,lift,yes,dumbbell,"biceps, forearm flexors",,
142,"curl, dumbbell, incline",no,lift,yes,"dumbbell, bench","biceps, forearm flexors",,
143,"curl, dumbbell, preacher",no,lift,yes,"dumbbell, preacher bench","biceps, forearm flexors",,
144,"curl, machine",no,lift,yes,bicep curl machine,"biceps, forearm flexors",,
145,"curl, spider",no,lift,yes,"dumbbell, bench","biceps, forearm flexors",,
146,dead bug,no,bodyweight,no,none,"abdominals, obliques",,
147,"deadlift, barbell",no,lift,yes,barbell,"glutes, lower back, quadriceps, hamstrings, adductors, trapezius, forearm flexors",,goal
148,"deadlift, barbell, romanian",no,lift,yes,barbell,"glutes, lower back, hamstrings, adductors, trapezius, forearm flexors",,
149,"deadlift, barbell, romanian, single-legged",yes,lift,yes,barbell,"glutes, lower back, hamstrings, adductors, trapezius, forearm flexors",,
150,"deadlift, dumbbell",no,lift,yes,dumbbell,"glutes, lower back, quadriceps, hamstrings, adductors, trapezius, forearm flexors",,
151,"deadlift, dumbbell, romanian",no,lift,yes,dumbbell,"glutes, lower back, hamstrings, adductors, trapezius, forearm flexors",,
152,"deadlift, stiff-legged",no,lift,yes,barbell,"glutes, lower back, hamstrings, adductors, trapezius, forearm flexors",,
153,down dog,no,bodyweight,no,none,"calves, front deltoid, rear deltoid, rotator cuffs, side deltoid, lower back, erector spinae, lats, abdominals, obliques, glutes, hamstrings",,
154,dragon flag,no,bodyweight,yes,stabilizer,"abdominals, obliques, glutes",,goal
155,face pull,no,lift,yes,cable machine,"rear deltoid, rotator cuffs, trapezius, side deltoid",,
156,floor front raise,no,bodyweight,no,none,lower back,,
157,"front raise, barbell",no,lift,yes,barbell,"front deltoid, side deltoid",,
158,"front raise, dumbbell",no,lift,yes,dumbbell,"front deltoid, side deltoid",,
159,"front raise, plate",no,lift,yes,weight plate,"front deltoid, side deltoid",,
160,glute bridge,no,bodyweight,no,none,"glutes, quadriceps",,
161,"glute kickback, machine, standing",yes,lift,yes,standing glute kickback machine,"glutes, hamstrings, adductors",,
162,"glute kickbacks, machine",yes,lift,yes,glute kickback machine,"glutes, hamstrings, adductors",,
163,good morning,no,lift,yes,barbell,"glutes, lower back, hamstrings, adductors",,
164,heel raise,no,bodyweight,yes,step,calves,,
165,"hip thrusts, barbell",no,lift,yes,"barbell, bench","glutes, adductors, quadriceps",,
166,"hip thrusts, machine",no,lift,yes,hip thrust machine,"abductors, glutes, quadriceps",,
167,knee raise,no,bodyweight,yes,pullup bar,"abdominals, obliques",,
168,"lateral raise, cable",yes,lift,yes,cable machine,"side deltoid, front deltoid",,
169,"lateral raise, dumbbell",no,lift,yes,dumbbell,"side deltoid, front deltoid",,
170,"lateral raise, machine",no,lift,yes,lateral raise machine,"side deltoid, front deltoid",,
171,leg extension,no,lift,yes,leg extension machine,quadriceps,,
172,leg press,no,lift,yes,leg press machine,"quadriceps, glutes, adductors, hamstrings",,
173,"lunge, barbell",no,lift,yes,"barbell, rack","quadriceps, glutes, adductors",,
174,"lunge, dumbbell",yes,lift,yes,dumbbell,"quadriceps, glutes, adductors, forearm flexors",,
175,lunges,yes,bodyweight,no,none,"quadriceps, glutes, adductors",,
176,"lunges, side",yes,bodyweight,no,none,"quadriceps, glutes, adductors",,
177,lying leg curl,no,lift,yes,lying leg curl machine,hamstrings,,
178,"overhead press, barbell",no,lift,yes,"barbell, rack","front deltoid, triceps, side deltoid",,goal
179,"overhead press, barbell, back",no,lift,yes,"barbell, rack","front deltoid, side deltoid, trapezius, triceps",,
180,"overhead press, barbell, seated",no,lift,yes,"barbell, bench","front deltoid, triceps, side deltoid",,
181,"overhead press, dumbbell",no,lift,yes,dumbbell,"front deltoid, triceps, side deltoid",,goal
182,"overhead press, dumbbell, seated",no,lift,yes,"dumbbell, bench","front deltoid, triceps, side deltoid",,
183,"overhead press, machine",no,lift,yes,shoulder press machine,"front deltoid, triceps, side deltoid",,
184,"overhead press, smith machine",no,lift,yes,smith machine,"front deltoid, triceps, side deltoid",,
185,"overhead press, smith machine, seated",no,lift,yes,"smith machine, bench","front deltoid, triceps, side deltoid",,
186,pec fly,no,lift,yes,pec fly machine,"pectorals, front deltoid",,
187,plank,no,bodyweight,no,none,"abdominals, pectorals, rhomboids, trapezius, lats",,
188,"plank, forearm",no,bodyweight,no,none,"abdominals, obliques",,
189,"plank, kneeling",no,bodyweight,no,none,"abdominals, obliques",,
190,"plank, kneeling, side",no,bodyweight,no,none,"abdominals, obliques",,
191,"plank, side",yes,bodyweight,no,none,"abdominals, obliques",,
192,prostrate pose,no,stretch,no,none,"front deltoid, rear deltoid, side deltoid, trapezius",,
193,push press,no,lift,yes,"barbell, rack","front deltoid, quadriceps, glutes, adductors, lower back, trapezius, triceps, side deltoid",,
194,pushup,no,bodyweight,no,none,"pectorals, front deltoid, triceps, abdominals",,goal
195,"pushup, clapping",no,bodyweight,no,none,"pectorals, front deltoid, triceps, abdominals",,
196,"pushup, close grip",no,bodyweight,no,none,"pectorals, front deltoid, triceps, abdominals",,
197,"pushup, diamond",no,bodyweight,no,none,"pectorals, front deltoid, triceps, abdominals",,
198,"pushup, incline",no,bodyweight,yes,step,"pectorals, front deltoid, triceps, abdominals",,
199,"pushup, kneeling",no,bodyweight,no,none,"pectorals, front deltoid, triceps, abdominals",,
200,"pushup, kneeling, incline",no,bodyweight,yes,step,"pectorals, front deltoid, triceps, abdominals",,
201,"pushup, wall",no,bodyweight,yes,wall,"pectorals, front deltoid, triceps, abdominals",,
202,"pushup, wide arm",no,bodyweight,no,none,"pectorals, front deltoid, triceps, abdominals",,
203,"row, barbell",no,lift,yes,barbell,"lats, trapezius, rear deltoid, biceps, lower back, forearm flexors, rotator cuffs",,goal
204,"row, barbell, upright",no,lift,yes,barbell or light barbell,"side deltoid, front deltoid, trapezius, biceps",,
205,"row, cable, rear delt",no,lift,yes,cable machine,"rear deltoid, rotator cuffs, trapezius, biceps",,
206,"row, cable, seated",no,lift,yes,cable machine,"lats, trapezius, rear deltoid, biceps, forearm flexors, rotator cuffs",,goal
207,"row, machine, seated",no,lift,yes,row machine,"lats, trapezius, rear deltoid, biceps, forearm flexors, rotator cuffs",,
208,rowing machine,no,cardio,yes,rowing machine,"quadriceps, glutes, adductors, hamstrings, calves, lats, trapezius, abdominals, lower back, front deltoid, rear deltoid, rotator cuffs, side deltoid, biceps",,
209,"run, track",no,cardio,no,none,"glutes, quadriceps, hamstrings, calves, ankles, abdominals",,goal
210,"run, treadmill",no,cardio,yes,treadmill,"glutes, quadriceps, hamstrings, calves, ankles, abdominals",,
211,russian twists,no,bodyweight,no,none,"abdominals, obliques",,
212,"shrug, barbell",no,lift,yes,barbell,"trapezius, forearm flexors",,
213,"shrug, dumbbell",no,lift,yes,dumbbell,"trapezius, forearm flexors",,
214,sit up,no,bodyweight,no,none,"abdominals, obliques",,
215,"sit up, hanging",no,bodyweight,yes,hanging sit up bench,"abdominals, obliques",,
216,"sit up, oblique",yes,bodyweight,no,none,"abdominals, obliques",,
217,"sit, L",no,bodyweight,no,none or paralettes,"abdominals, obliques, quadriceps, hamstrings",,goal
218,"sit, V",no,bodyweight,yes,none or paralettes,"abdominals, obliques, quadriceps, hamstrings, abductors, adductors",,goal
219,"squat, barbell",no,lift,yes,"barbell, rack","quadriceps, adductors, glutes, lower back, calves",,goal
220,"squat, barbell, front",no,lift,yes,"barbell, rack","quadriceps, glutes, adductors, lower back, calves",,
221,"squat, barbell, hack",no,lift,yes,barbell,"quadriceps, glutes, adductors, calves, lower back, trapezius, forearm flexors",,
222,"squat, bulgarian split",yes,lift,yes,"barbell, bench","quadriceps, glutes, adductors",,
223,"squat, dumbbell",no,lift,yes,dumbbell,"quadriceps, glutes, adductors, lower back, calves",,
224,"squat, sissy",no,bodyweight,no,none,"quadriceps, abdominals, ankles",,
225,"squat, smith machine",no,lift,yes,smith machine,"quadriceps, glutes, adductors, lower back, calves",,
226,"squats, hack squat machine",no,lift,yes,hack squat machine,"quadriceps, glutes, adductors",,
227,toe touches,no,bodyweight,no,none,"abdominals, obliques",,
228,"dead hang, supinated",no,bodyweight,yes,pullup bar,"forearm flexors, trapezius, triceps, biceps, rhomboids, front deltoid, rear deltoid, abdominals, obliques",pullup,
229,"dead hang, one arm, supinated",yes,bodyweight,yes,pullup bar,"forearm flexors, trapezius, triceps, biceps, rhomboids, front deltoid, rear deltoid, abdominals, obliques",pullup,
    "#;
