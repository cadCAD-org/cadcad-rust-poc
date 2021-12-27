
# import libraries
from cadCAD.utils import print_pipe
import pandas as pd
import numpy as np
import matplotlib 
from cadCAD.engine import ExecutionMode, ExecutionContext, Executor
# import config
import config_pp
from cadCAD import configs
import matplotlib.pyplot as plt
import time

## 
start = time.process_time()
##
exec_mode = ExecutionMode()
first_config = configs # only contains config1
single_proc_ctx = ExecutionContext(context=exec_mode.single_proc)
run = Executor(exec_context=single_proc_ctx, configs=first_config)
raw_result, tensor_field = run.execute()
##
end = time.process_time()
print(end - start, "sec(s)")

## Print
print_result = 0
if print_result:
  for i in raw_result:
    print(i)

##
# df = pd.DataFrame(raw_result)
# df.set_index(['run', 'timestep', 'substep'])
# df.plot('timestep', ['box_A', 'box_B'], grid=True, 
#         colormap = 'RdYlGn',
#         xticks=list(df['timestep'].drop_duplicates()), 
#         yticks=list(range(1+(df['box_A']+df['box_B']).max())));