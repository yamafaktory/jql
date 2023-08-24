| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 23.8 ± 0.3 | 23.3 | 29.1 | 14.02 ± 1.18 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 1.7 ± 0.1 | 1.5 | 2.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 23.6 ± 0.2 | 23.1 | 25.5 | 14.16 ± 1.33 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 1.7 ± 0.2 | 1.5 | 2.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 23.6 ± 0.8 | 23.1 | 44.7 | 15.15 ± 1.58 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 1.6 ± 0.2 | 1.4 | 3.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 147.0 ± 1.4 | 145.9 | 178.5 | 7.48 ± 1.05 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 19.7 ± 2.8 | 16.3 | 60.5 | 1.00 |

