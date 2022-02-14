
from cadCAD.engine import ExecutionMode, ExecutionContext, Executor
import config_prey_predator
from cadCAD import configs
import time

## 
start = time.process_time()
##
exec_mode = ExecutionMode()
first_config = configs
single_proc_ctx = ExecutionContext(context=exec_mode.single_proc)
run = Executor(exec_context=single_proc_ctx, configs=first_config)
raw_result, tensor_field = run.execute()
##
end = time.process_time()
print("Simulation took", end - start, "sec(s)")

## Print trajectory
print_result = 0
if print_result:
  for i in raw_result:
    print(i)