| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 32.5 ± 0.6 | 31.3 | 34.6 | 19.31 ± 1.75 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 1.7 ± 0.1 | 1.5 | 2.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 32.5 ± 0.6 | 31.3 | 35.0 | 19.07 ± 1.96 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 1.7 ± 0.2 | 1.5 | 2.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 32.2 ± 0.6 | 31.3 | 36.6 | 19.29 ± 1.64 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 1.7 ± 0.1 | 1.5 | 2.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 164.6 ± 1.2 | 163.3 | 177.4 | 1.26 ± 0.03 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 130.5 ± 2.4 | 126.2 | 168.9 | 1.00 |

