import optuna
import argparse
from datetime import datetime
import os
import test


def objective(trial):
    params = {
        'SAMPLE_PARAM': trial.suggest_int('SAMPLE_PARAM', -300, 300),
    }
    env_params = [f'{k}={v}' for k, v in params.items()]
    scores_inds = test.run_test(0, 10, envs=env_params, remove_ans_file=True, features=["env_const"], stdout_build=False)
    scores = [v[0] for v in scores_inds]
    if not scores:
        ave = 0
    else:
        ave = sum(scores)/len(scores)
    print(ave)
    return ave


parser = argparse.ArgumentParser()
parser.add_argument('--name', type=str, default='')
args = parser.parse_args()
name = args.name
if name == '':
    name = datetime.now().strftime('%Y_%m%d_%H%M%S')

# optuna-dashboard sqlite:///tools/optuna_study.db
storage = f'sqlite:///{os.path.dirname(__file__)}/optuna_study.db'

study = optuna.create_study(direction='maximize', study_name=name, storage=storage, load_if_exists=True)
study.optimize(objective, n_trials=100)
print(study.best_params)
print(study.best_value)
