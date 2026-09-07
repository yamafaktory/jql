| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 2.6 ± 0.1 | 2.5 | 3.1 | 1.00 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 3.1 ± 0.6 | 2.4 | 6.9 | 1.19 ± 0.25 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 2.5 ± 0.3 | 2.4 | 11.4 | 1.00 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 3.1 ± 0.6 | 2.4 | 7.0 | 1.23 ± 0.29 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 2.5 ± 0.1 | 2.4 | 3.2 | 1.00 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 3.1 ± 0.6 | 2.4 | 7.2 | 1.23 ± 0.23 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 95.5 ± 6.0 | 88.6 | 133.6 | 5.29 ± 0.51 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.1 ± 1.3 | 14.8 | 26.7 | 1.00 |

