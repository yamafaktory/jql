| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 19.7 ± 0.4 | 19.1 | 26.4 | 6.99 ± 0.35 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.8 ± 0.1 | 2.3 | 3.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 19.8 ± 0.3 | 19.0 | 21.4 | 7.00 ± 0.33 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.8 ± 0.1 | 2.4 | 3.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 19.8 ± 0.5 | 19.2 | 29.4 | 7.00 ± 0.36 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.8 ± 0.1 | 2.4 | 3.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 121.5 ± 5.4 | 114.9 | 163.9 | 6.57 ± 0.36 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.5 ± 0.6 | 16.8 | 23.0 | 1.00 |

