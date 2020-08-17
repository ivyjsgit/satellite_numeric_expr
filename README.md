# Satellite numeric expr

## What is this project?

This project is the executable wrapper for [satellite_numeric](https://github.com/ivyjsgit/satellite_numeric)!

It is used to run the anytime planner, along with the experiments used in the research project.

## How do I run the experiments?

There are two ways to run the experiments: You can either manually run them by running the command

> cargo run -- -15s -3v pfilepath

Where 15s is replaced with the maximum duration you wish the experiments to run for, and 3 being the verbosity. 

4 is the most verbose, 0 is the least, with 2 being "just right" for set it and forget it automation without spamming the log file.

The second way to run experiments is to first place your experiment files in the location ../Numeric/ and then run 
> python3 experiment_automator.py 

This will set the timeout to 15 seconds, and the verbosity to 2, and will run the experiments on pfile[1-20]. From there, you can move the results to a seperate folder.
