| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 19.9 ± 1.1 | 19.3 | 48.2 | 7.21 ± 0.51 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.8 ± 0.1 | 2.5 | 3.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 19.8 ± 0.7 | 19.2 | 38.2 | 7.30 ± 0.43 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.7 ± 0.1 | 2.4 | 3.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 19.8 ± 0.7 | 19.3 | 35.1 | 7.26 ± 0.48 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.7 ± 0.2 | 2.4 | 4.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 120.3 ± 4.7 | 115.1 | 161.0 | 6.66 ± 0.39 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.1 ± 0.8 | 16.4 | 33.6 | 1.00 |

