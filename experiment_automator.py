import os
for i in range(1,21):
    experiment_name="pfile"+ str(i)
    command="cargo run -- -15s -2v ../Numeric/"+experiment_name
    print(command)
    os.popen(command).read()
