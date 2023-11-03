| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 40.3 ± 3.4 | 37.0 | 68.2 | 11.07 ± 2.84 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 3.6 ± 0.9 | 2.6 | 17.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 39.9 ± 2.2 | 37.0 | 52.7 | 11.71 ± 2.88 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 3.4 ± 0.8 | 2.4 | 13.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 40.3 ± 2.0 | 37.2 | 52.2 | 12.24 ± 3.10 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 3.3 ± 0.8 | 2.3 | 14.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 199.5 ± 5.5 | 188.6 | 240.5 | 6.58 ± 1.01 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 30.3 ± 4.6 | 23.5 | 89.2 | 1.00 |

