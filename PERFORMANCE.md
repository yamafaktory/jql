| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 27.2 ± 0.6 | 26.0 | 29.8 | 10.42 ± 1.04 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.6 ± 0.3 | 2.0 | 5.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 27.0 ± 0.5 | 25.7 | 28.6 | 11.08 ± 1.08 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.4 ± 0.2 | 2.0 | 4.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 27.2 ± 0.6 | 26.0 | 29.5 | 10.77 ± 1.13 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.5 ± 0.3 | 2.1 | 7.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 157.9 ± 1.4 | 156.2 | 191.5 | 7.03 ± 0.60 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 22.5 ± 1.9 | 18.5 | 37.0 | 1.00 |

