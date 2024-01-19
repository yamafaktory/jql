| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 20.0 ± 0.3 | 19.4 | 25.7 | 7.26 ± 0.39 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.7 ± 0.1 | 2.4 | 3.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 19.9 ± 0.4 | 19.3 | 25.3 | 7.17 ± 0.39 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.8 ± 0.1 | 2.4 | 3.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 19.8 ± 0.9 | 19.1 | 36.9 | 7.25 ± 0.52 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.7 ± 0.2 | 2.4 | 4.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 121.4 ± 5.4 | 115.5 | 165.5 | 6.64 ± 0.34 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.3 ± 0.5 | 16.8 | 20.6 | 1.00 |

