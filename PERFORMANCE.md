| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 2.5 ± 0.1 | 2.4 | 3.2 | 1.16 ± 0.06 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.2 ± 0.1 | 2.0 | 4.0 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 2.5 ± 0.1 | 2.4 | 3.6 | 1.15 ± 0.09 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.2 ± 0.2 | 2.0 | 6.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 2.5 ± 0.1 | 2.4 | 4.1 | 1.14 ± 0.09 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.2 ± 0.1 | 2.0 | 4.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 91.5 ± 4.9 | 86.9 | 120.1 | 5.14 ± 0.32 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 17.8 ± 0.6 | 15.7 | 24.4 | 1.00 |

