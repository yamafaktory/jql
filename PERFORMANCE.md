| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 23.7 ± 0.3 | 23.1 | 24.9 | 14.18 ± 1.13 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 1.7 ± 0.1 | 1.5 | 2.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 23.5 ± 0.2 | 23.0 | 25.3 | 14.18 ± 1.25 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 1.7 ± 0.1 | 1.5 | 2.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 23.4 ± 0.3 | 22.9 | 30.4 | 15.42 ± 1.46 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 1.5 ± 0.1 | 1.4 | 2.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 146.5 ± 0.7 | 145.5 | 153.0 | 7.62 ± 0.92 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 19.2 ± 2.3 | 15.5 | 44.8 | 1.00 |

