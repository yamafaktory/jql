| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 20.2 ± 0.6 | 19.3 | 30.2 | 7.22 ± 0.54 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.8 ± 0.2 | 2.4 | 4.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 20.3 ± 0.3 | 19.4 | 22.7 | 7.31 ± 0.49 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.8 ± 0.2 | 2.4 | 3.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 20.0 ± 0.4 | 19.0 | 22.0 | 7.19 ± 0.47 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.8 ± 0.2 | 2.3 | 3.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 121.2 ± 4.8 | 115.7 | 176.0 | 6.37 ± 0.39 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 19.0 ± 0.9 | 17.0 | 34.6 | 1.00 |

