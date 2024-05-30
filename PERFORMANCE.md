| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 19.9 ± 0.3 | 19.2 | 22.7 | 7.20 ± 0.71 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.8 ± 0.3 | 2.5 | 9.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 19.8 ± 0.4 | 19.2 | 28.5 | 7.24 ± 0.52 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.7 ± 0.2 | 2.4 | 5.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 19.7 ± 0.4 | 19.2 | 23.3 | 7.29 ± 0.68 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.7 ± 0.2 | 2.3 | 6.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 121.4 ± 5.6 | 115.0 | 206.7 | 6.63 ± 0.36 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.3 ± 0.5 | 16.8 | 22.2 | 1.00 |

