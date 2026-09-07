| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 2.6 ± 0.1 | 2.5 | 3.2 | 1.00 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 3.0 ± 0.6 | 2.4 | 7.2 | 1.18 ± 0.23 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 2.5 ± 0.0 | 2.4 | 2.8 | 1.00 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 3.0 ± 0.7 | 2.4 | 7.6 | 1.23 ± 0.27 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 2.5 ± 0.1 | 2.4 | 3.8 | 1.00 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 3.0 ± 0.6 | 2.4 | 6.8 | 1.21 ± 0.23 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 94.4 ± 6.4 | 88.1 | 127.0 | 5.24 ± 0.50 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.0 ± 1.2 | 15.3 | 26.7 | 1.00 |

