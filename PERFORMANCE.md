| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 19.9 ± 0.3 | 19.2 | 22.1 | 7.22 ± 0.47 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.8 ± 0.2 | 2.4 | 4.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 19.7 ± 0.4 | 19.2 | 24.1 | 7.40 ± 0.49 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.7 ± 0.2 | 2.4 | 4.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 19.6 ± 0.9 | 19.0 | 46.2 | 7.23 ± 0.79 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.7 ± 0.3 | 2.4 | 9.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 121.1 ± 5.0 | 115.3 | 156.0 | 6.63 ± 0.45 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.3 ± 1.0 | 16.5 | 36.2 | 1.00 |

