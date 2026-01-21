| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 2.5 ± 0.1 | 2.4 | 3.0 | 1.16 ± 0.04 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.2 ± 0.1 | 2.0 | 2.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 2.5 ± 0.1 | 2.4 | 3.7 | 1.14 ± 0.04 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.2 ± 0.0 | 2.0 | 2.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 2.5 ± 0.1 | 2.4 | 3.8 | 1.14 ± 0.05 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.2 ± 0.0 | 2.0 | 2.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 93.3 ± 5.4 | 86.9 | 131.7 | 5.13 ± 0.36 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.2 ± 0.7 | 16.2 | 28.0 | 1.00 |

