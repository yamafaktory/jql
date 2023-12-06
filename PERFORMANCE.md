| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 20.5 ± 0.4 | 19.6 | 29.1 | 6.73 ± 0.40 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 3.0 ± 0.2 | 2.5 | 3.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 20.4 ± 0.7 | 19.6 | 31.0 | 6.36 ± 2.85 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 3.2 ± 1.4 | 2.6 | 39.0 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 20.3 ± 0.3 | 19.5 | 23.2 | 6.72 ± 0.39 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 3.0 ± 0.2 | 2.6 | 3.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 123.4 ± 5.6 | 116.5 | 178.2 | 6.18 ± 0.34 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 20.0 ± 0.6 | 18.1 | 25.7 | 1.00 |

