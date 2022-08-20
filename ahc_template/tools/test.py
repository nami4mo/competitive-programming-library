import subprocess
import os
from datetime import datetime
import argparse
import pyperclip
from tqdm import tqdm
import sys

ROOT = f'{os.environ["HOME"]}/Desktop/competitive-programming'
EXE_FILE = f'{ROOT}/target/release/ahc_template'
RUN = EXE_FILE
# for interactive problems
# RUN = f'./tools/tester {EXE_FILE}'
PRJ_DIR = f'{ROOT}/Library/ahc_template/'
MAX_PROC = 8


def run_test(l, r, envs=[], remove_ans_file=False, features=[], stdout_build=True):
    # ---------- build ----------
    os.chdir(PRJ_DIR)
    subprocess.run(f'rm {EXE_FILE}', shell=True)
    envs = ' '.join(envs)
    features = ' '.join(features + ["mylocal"])
    cmd = f'{envs} cargo build --release --features "{features}"'
    if stdout_build:
        subprocess.run(cmd, shell=True)
    else:
        subprocess.run(cmd, shell=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

    # ---------- make output dir ----------
    date = datetime.now().strftime('%Y_%m%d_%H%M%S')
    out_dir = f'./tools/out/{date}'
    tmp_score_out_dir = f'./tools/out/{date}/tmp'
    score_file = f'./tools/out/{date}/0_score.txt'
    subprocess.run(f'mkdir -p {out_dir}', shell=True)
    subprocess.run(f'mkdir -p {tmp_score_out_dir}', shell=True)

    # ---------- run ----------
    index_list = list(range(l, r+1))
    procs_list = []
    for ei, i in enumerate(tqdm(index_list)):
        filename = f'{i:04}.txt'
        cmd = f'{RUN} < ./tools/in/{filename} > {out_dir}/{i:04}.txt 2>> {tmp_score_out_dir}/{i:04}.txt'
        proc = subprocess.Popen(cmd, shell=True)
        procs_list.append((i, proc))
        if (ei + 1) % MAX_PROC == 0 or ei == len(index_list)-1:
            for _, subproc in procs_list:
                subproc.wait()
            procs_list = []

    # ---------- collect scores ----------
    scores = []
    scores_inds = []
    for i in index_list:
        with open(f'{tmp_score_out_dir}/{i:04}.txt', 'r') as f:
            rows = f.readlines()
            for v in rows:
                if 'Score = ' in v:
                    v = v.replace('Score = ', '')
                    try:
                        v = int(float(v))
                        scores.append(v)
                        scores_inds.append((v, i))
                        break
                    except:
                        print(f'{i:04}: Score is not Number', file=sys.stderr)
            else:
                print(f'{i:04}: No score.', file=sys.stderr)

    # ---------- output scores ----------
    with open(score_file, 'w') as f:
        for v in scores:
            f.write(str(v)+'\n')

    # ---------- copy the last ans or remove ans files ----------
    if remove_ans_file:
        subprocess.run(f'rm {out_dir}/*.txt', shell=True)
        subprocess.run(f'rm {tmp_score_out_dir}/*.txt', shell=True)
        subprocess.run(f'rmdir {tmp_score_out_dir}', shell=True)
        subprocess.run(f'rmdir {out_dir}', shell=True)
    else:
        ans = ''
        with open(f'{out_dir}/{index_list[-1]:04}.txt', 'r') as f:
            for row in f.readlines():
                ans += row.rstrip()
                ans += '\n'
            pyperclip.copy(ans)

    # ---------- return result ----------
    return scores_inds


if __name__ == '__main__':
    # ---------- parse args ----------
    parser = argparse.ArgumentParser()
    parser.add_argument('left', type=int)
    parser.add_argument('right', type=int)
    # other params
    # parser.add_argument('-p', '--pattern', type=int, default=-1)
    args = parser.parse_args()
    l, r = args.left, args.right
    scores_inds = run_test(l, r, remove_ans_file=False, features=[])
    scores = [v[0] for v in scores_inds]

    ave = sum(scores)/len(scores)
    print(f'average: {int(ave)}')
    scores_inds.sort(key=lambda x: x[0])
    print('# worst')
    for score, ind in scores_inds[:min(10, len(scores_inds))]:
        print(f'{ind:04}: {score}')
