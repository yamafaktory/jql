| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 2.5 ± 0.1 | 2.4 | 3.6 | 1.17 ± 0.09 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.2 ± 0.2 | 2.0 | 6.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 2.5 ± 0.2 | 2.4 | 7.2 | 1.16 ± 0.09 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.1 ± 0.1 | 2.0 | 3.0 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 2.5 ± 0.2 | 2.3 | 6.9 | 1.16 ± 0.16 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.2 ± 0.2 | 2.0 | 9.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 90.8 ± 4.5 | 86.7 | 126.5 | 5.29 ± 0.36 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 17.2 ± 0.8 | 15.4 | 23.0 | 1.00 |

