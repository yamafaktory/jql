| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 2.1 ± 0.1 | 2.0 | 2.6 | 1.51 ± 0.06 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 1.4 ± 0.0 | 1.3 | 1.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 2.0 ± 0.1 | 1.9 | 2.5 | 1.47 ± 0.06 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 1.4 ± 0.0 | 1.3 | 1.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 2.0 ± 0.1 | 1.9 | 3.2 | 1.46 ± 0.07 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 1.4 ± 0.0 | 1.3 | 1.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 80.9 ± 4.7 | 76.3 | 97.3 | 4.38 ± 0.31 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.5 ± 0.8 | 16.0 | 21.0 | 1.00 |

