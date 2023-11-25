| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 19.8 ± 0.3 | 19.1 | 26.6 | 7.05 ± 0.39 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.8 ± 0.1 | 2.3 | 3.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 19.7 ± 0.4 | 19.1 | 29.0 | 6.98 ± 0.40 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.8 ± 0.2 | 2.4 | 3.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 19.7 ± 0.8 | 18.9 | 29.1 | 7.04 ± 0.46 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.8 ± 0.1 | 2.4 | 3.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 120.6 ± 5.0 | 114.9 | 167.0 | 6.46 ± 0.39 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.6 ± 0.8 | 16.9 | 32.0 | 1.00 |

