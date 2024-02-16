1)  `./judgeops -m db.json apply-judgement tests/post_audit.report-config.judge.md`

It creates db.json if not exists

It looks at all the stuff inside post_audit.report-config.judge.md and it has now everything needed to register all detectors

It applies judgement and stores feedback in specified db.json

2) `./judgeops -m db.json display-metrics`
Scans db.json and prints scoreboard 

3) `./judgeops -m db.json display-metrics <detector_name>`
Scans db.json and prints stats specific to that detector

4) `./judgeops -m db.json give-opinion`
Scans db.json and gives opinion just like lightchaser (so it sees and reports if any detector has fallen below lower ceiling, or has reached 0 and needs to be disqualified, etc, etc)


