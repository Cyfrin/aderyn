## Guide to use `judgeops`

### It is currently built for helping with quality assurance specially for  core detectors. 

##### *You can still follow along to see how `nyth` detectors can be incorporated 

START LOOP


--------

#### STEP 1

Create a new branch in aderyn codebase for each new season
	Example -> competition/season-1

--------

#### STEP 2

Bring all the `nyth` detectors that have passed some basic tests  into this codebase and add them to the list inside `fn get_all_issue_detectors()` 

Also do not forget to give each one of these names from the `DetectorNamePool`  (of course you have to create new names )

(Basically what we have done is introduced new detectors in the core)

--------

#### STEP 3

From the root directory of aderyn repo, run

`cargo run --bin judgeops -- -d`

You will notice that it fails with Exit Code 1 and in doing so it will also print out all of the new detectors in the core repo that watchtower has no idea about.

Here is why this is important - Any potential feedback submitted to it is assumed to come from the set of latest core detectors. So if something's missing watchtower could enter panic mode! Therefore `judgeops` will absolutely prevent this before hand by forcing you to acknowledge all the new ones. (see the next step for how)

This command is also run in our CI tests under "Check Watchtotwer Health" 

So you can never push code that can reach potentital "panickable" watchtower state : )

--------

#### STEP 4

From the root directory of aderyn repo, run

`cargo run --bin judgeops -- -a`

This will **a**utomatically register all of the extra detectors that are added to core repo but are not in the database. Here, these would be the 	`nyth` detectors that we just imported

--------

#### STEP 5

Now you can submit feedbacks  (either as json as shown in `metricsdbgen.sh`) or as markdown (shown below here)

You will see the affected changes printed to screen as you do this

`cargo run --bin judgeops give-feedback postaudit.abc.judge.md`

Repeat Step 5 a good couple of times -  more feedback, the better!!!

--------


#### STEP 6

Run

`cargo run --bin judgeops -- -s` 

to print **s**uggested changes

Decide if you want to get rid of (a.k.a unregister)  any bad detector printed to screen as a result of running above command -

Unregister command:
 
`cargo run --bin judgeops -- -u <detector-name>`

Now, also remove the detector from `get_all_issue_detectors` list 

Now if you want you can make some adjustments like changing `IssueSeverity` etc, and then re add it to the `get_all_issue_detectors` list followed by `cargo run --bin judgeops -- -a` to re-enlist the detector as a brand new detector.

(Once you make a change to the detector there is no guarantee all of it's previous metrics hold true or are of any value unless explicitly re-tested) 

--------

#### STEP 7

Save the database
Run Git Commit

--------

#### STEP 6

To get the rankings,

For each newly registered detector run this

`cargo run --bin judgeops -- display-metrics <detector-name>`

It will print the Rating

(This can be tedious - I have an idea around it; will try to file a new PR. Basically display metrics for all the registered detectors in descending order of the Detector rating)

That can then serve as the leaderboard for that season. 

------

#### STEP 9
Merge to dev! 😄 

END LOOP 