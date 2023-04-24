| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 23.7 ± 0.5 | 23.1 | 29.4 | 13.96 ± 2.20 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 1.7 ± 0.3 | 1.5 | 6.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 23.5 ± 0.4 | 22.9 | 29.1 | 14.40 ± 1.63 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 1.6 ± 0.2 | 1.4 | 4.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 23.7 ± 0.9 | 22.9 | 32.7 | 14.55 ± 1.88 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 1.6 ± 0.2 | 1.4 | 5.0 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 145.9 ± 1.6 | 144.8 | 185.9 | 8.18 ± 0.93 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 17.8 ± 2.0 | 15.4 | 40.1 | 1.00 |

