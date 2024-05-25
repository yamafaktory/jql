| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 19.9 ± 0.5 | 19.2 | 29.4 | 7.23 ± 0.50 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.7 ± 0.2 | 2.3 | 3.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 19.8 ± 0.4 | 19.1 | 26.3 | 7.35 ± 0.47 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.7 ± 0.2 | 2.4 | 3.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 19.8 ± 0.4 | 19.1 | 22.9 | 7.51 ± 0.52 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.6 ± 0.2 | 2.2 | 4.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 121.5 ± 4.9 | 115.2 | 151.7 | 6.45 ± 0.33 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.8 ± 0.6 | 16.7 | 24.9 | 1.00 |

