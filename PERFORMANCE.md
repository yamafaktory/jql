| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 20.0 ± 0.5 | 19.3 | 31.8 | 7.33 ± 0.44 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.7 ± 0.1 | 2.4 | 3.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 19.9 ± 0.4 | 19.3 | 30.7 | 7.31 ± 0.71 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.7 ± 0.3 | 2.4 | 9.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 19.9 ± 0.6 | 19.1 | 30.7 | 7.70 ± 0.51 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.6 ± 0.2 | 2.2 | 3.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 121.8 ± 4.7 | 115.9 | 138.7 | 6.59 ± 0.32 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.5 ± 0.5 | 16.7 | 22.6 | 1.00 |

