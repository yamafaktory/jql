| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 20.7 ± 0.5 | 19.7 | 27.5 | 7.24 ± 0.69 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.9 ± 0.3 | 2.4 | 4.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 20.6 ± 0.5 | 19.5 | 24.9 | 7.17 ± 0.60 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.9 ± 0.2 | 2.3 | 4.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 20.5 ± 0.5 | 19.3 | 26.7 | 7.21 ± 0.60 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.8 ± 0.2 | 2.3 | 4.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 123.5 ± 5.5 | 116.1 | 163.4 | 6.59 ± 0.38 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.7 ± 0.7 | 16.6 | 24.1 | 1.00 |

