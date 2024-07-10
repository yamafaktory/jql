| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 20.0 ± 1.0 | 19.1 | 35.7 | 7.50 ± 0.60 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.7 ± 0.2 | 2.4 | 4.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 19.8 ± 0.3 | 19.2 | 23.0 | 7.36 ± 0.56 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.7 ± 0.2 | 2.3 | 6.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 19.7 ± 0.7 | 19.0 | 35.8 | 7.43 ± 0.68 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.7 ± 0.2 | 2.4 | 8.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 121.6 ± 5.3 | 115.1 | 159.1 | 6.69 ± 0.38 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.2 ± 0.7 | 16.7 | 29.4 | 1.00 |

