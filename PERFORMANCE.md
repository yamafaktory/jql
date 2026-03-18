| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 2.9 ± 0.2 | 2.6 | 4.2 | 1.03 ± 0.10 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.8 ± 0.2 | 2.3 | 3.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 2.9 ± 0.2 | 2.6 | 4.1 | 1.00 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 3.1 ± 0.2 | 2.6 | 3.8 | 1.06 ± 0.09 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 2.9 ± 0.2 | 2.6 | 4.3 | 1.00 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 3.0 ± 0.2 | 2.5 | 3.6 | 1.02 ± 0.09 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 98.5 ± 6.3 | 89.2 | 168.2 | 6.27 ± 0.46 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 15.7 ± 0.5 | 14.4 | 18.8 | 1.00 |

