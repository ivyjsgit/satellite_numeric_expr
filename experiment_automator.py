import os
import threading
import subprocess
import sys

def runCommand(command):
    os.popen(command).read()

def worker(min, max):
    for i in range(min,max):
        experiment_name="pfile"+ str(i)
        command="cargo run -- -15s -2v ../Numeric/"+experiment_name
        print(command)
        runCommand(command)

worker1 = threading.Thread(target=worker, args=([1,5])).start()
worker2 = threading.Thread(target=worker, args=([6,10])).start()
worker3 = threading.Thread(target=worker, args=([11,15])).start()
worker4 = threading.Thread(target=worker,args=([16,21])).start()
