from ntpath import join
from pathlib import Path
import re
import subprocess
from enum import Enum
import argparse
import yaml
import codecs


BEGIN_DOC = '@doc.begin'
END_DOC = '@doc.end'

BEGIN_SRC = '@doc.src.begin'
END_SRC = '@doc.src.end'

BEGIN_SRC_COLLAPSE = '@doc.src_c.begin'
END_SRC_COLLAPSE = '@doc.src_c.end'

BEGIN_TEXT = '@doc.text.begin'
END_TEXT = '@doc.text.end'

INLINE_TEXT = '@doc.text.inline'
INLINE_TITLE = '@doc.title'
INLINE_SUBTITLE = '@doc.subtitle'

DOCS_ROOT = '../docs'
DOCS_MD_DIR = f'{DOCS_ROOT}/docs'
MKDOCS_YML_TEMPLATE = f'{DOCS_ROOT}/mkdocs_template.yml'
MKDOCS_YML = f'{DOCS_ROOT}/mkdocs.yml'


class Docs:
    class Mode(Enum):
        NOTHING = 1
        TEXT = 2
        SRC = 3

    def __init__(self) -> None:
        self._doc_str_dic = {}

    @staticmethod
    def _get_doc_name(row: str) -> str:
        '''
        fuga {hoge} [yeah] -> year
        '''
        ls = re.findall("(?<=\[).+?(?=\])", row)
        if ls:
            return ls[0]
        else:
            ''

    @staticmethod
    def _get_doc_title(row: str) -> str:
        '''
        fuga {hoge} [yeah] -> hoge
        '''
        ls = re.findall("(?<=\{).+?(?=\})", row)
        if ls:
            return ls[0]
        else:
            ''

    @staticmethod
    def _insert_src_end(doc_str: list[str]) -> None:
        if not doc_str:
            return
        if '```' in doc_str[-1]:
            doc_str.pop()
        else:
            doc_str.append('```\n\n')

    def insert_file_to_doc_str(self, file_p: Path) -> None:
        ext = file_p.suffix.lstrip('.')
        with open(file_p, 'r') as f:
            rows = f.readlines()
            mode = self.Mode.NOTHING
            doc_name = ''
            for row in rows:
                if BEGIN_DOC in row:
                    doc_name = self._get_doc_name(row)
                    assert doc_name != '', f'no doc_name in [{file_p}]'
                    self._doc_str_dic.setdefault(doc_name, [])
                    title = self._get_doc_title(row)
                    if title:
                        self._doc_str_dic[doc_name].append(f'# {title}\n')
                elif END_DOC in row:
                    mode = self.Mode.NOTHING
                elif BEGIN_SRC in row:
                    mode = self.Mode.SRC
                    self._doc_str_dic[doc_name].append(f'\n```{ext}\n')
                elif END_SRC in row:
                    mode = self.Mode.NOTHING
                    self._doc_str_dic[doc_name].append(f'```\n')
                elif BEGIN_SRC_COLLAPSE in row:
                    mode = self.Mode.SRC
                    title = self._get_doc_title(row)
                    if not title:
                        title = 'Source Code'
                    self._doc_str_dic[doc_name].append(f'<details><summary>{title}</summary>\n\n```{ext}\n')
                elif END_SRC_COLLAPSE in row:
                    mode = self.Mode.NOTHING
                    self._doc_str_dic[doc_name].append(f'```\n\n</details>')
                elif BEGIN_TEXT in row:
                    mode = self.Mode.TEXT
                elif END_TEXT in row:
                    mode = self.Mode.NOTHING
                elif INLINE_TITLE in row:
                    title = self._get_doc_title(row)
                    if title:
                        self._doc_str_dic[doc_name].append(f'# {title}\n')
                elif INLINE_SUBTITLE in row:
                    title = self._get_doc_title(row)
                    if title:
                        self._doc_str_dic[doc_name].append(f'## {title}\n')
                elif INLINE_TEXT in row:
                    text = row[row.find(INLINE_TEXT)+len(INLINE_TEXT):].lstrip()
                    if text:
                        self._doc_str_dic[doc_name].append(f'{text}  ')
                elif mode == self.Mode.SRC:
                    self._doc_str_dic[doc_name].append(row)
                elif mode == self.Mode.TEXT:
                    self._doc_str_dic[doc_name].append(row.replace('\n', '  \n'))

    def make_md_docs(self):
        for key, value in self._doc_str_dic.items():
            # why lower() ?
            # -> if uppercase in dir name, mkdocs does not work well...(?)
            doc_dir = '/'.join(f'{DOCS_MD_DIR}/{key}'.split('/')[:-1]).lower()
            subprocess.run(f'mkdir -p {doc_dir}', shell=True)
            with open(f'{DOCS_MD_DIR}/{key}.md'.lower(), 'w') as f:
                doc_str = ''.join(value)
                f.write(doc_str)

    def make_mkdocs_yml(self):
        LIST_KEY = '_list'

        def _insert_dic_rec(dic: dict, keys: list[str]):
            if len(keys) == 1:
                dic.setdefault(LIST_KEY, [])
                dic[LIST_KEY].append(f'{keys[-1]}')
            else:
                dic.setdefault(keys[0], {})
                _insert_dic_rec(dic[keys[0]], keys[1:])

        def _construct_list_rec(dic: dict, key: str, keys: list[str]) -> list:
            res = []
            if key:
                keys.append(key)
            for k, v in dic.items():
                if k == LIST_KEY:
                    res.extend(map(lambda x: f'{"/".join(keys)}/{x}.md'.lstrip('/').lower(), v))
                else:
                    res.append({k: _construct_list_rec(v, k, keys)})
            dic[key] = res
            if key:
                keys.pop()
            return res

        with open(MKDOCS_YML_TEMPLATE, 'r') as f:
            mk_yml = yaml.safe_load(f)
        nav_dir = {}
        nav_list = []
        for key in self._doc_str_dic.keys():
            keys = key.split('/')
            _insert_dic_rec(nav_dir, keys)
        nav_list = _construct_list_rec(nav_dir, '', [])
        mk_yml['nav'] = nav_list
        with codecs.open(MKDOCS_YML, 'w', 'utf-8') as f:
            yaml.dump(mk_yml, f, encoding='utf-8', allow_unicode=True)


def get_all_files(in_dir: Path) -> list[Path]:
    p_files = in_dir.glob('**/*')
    return filter(lambda x: x.is_file(), p_files)


def main():
    parser = argparse.ArgumentParser(description='make document by mkdocs')
    parser.add_argument('in_dir', type=Path, help='source code directory')
    args = parser.parse_args()
    in_dir = Path(args.in_dir)
    p_files = get_all_files(in_dir)

    docs = Docs()
    for p_file in p_files:
        # print(p_file)
        docs.insert_file_to_doc_str(p_file)
    docs.make_md_docs()
    docs.make_mkdocs_yml()


if __name__ == '__main__':
    main()
