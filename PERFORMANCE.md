| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 26.6 ± 0.5 | 25.5 | 28.5 | 16.01 ± 1.58 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 1.7 ± 0.2 | 1.5 | 3.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 26.6 ± 0.6 | 25.4 | 28.8 | 15.91 ± 1.95 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 1.7 ± 0.2 | 1.5 | 5.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 26.5 ± 0.6 | 25.4 | 30.0 | 15.77 ± 2.30 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 1.7 ± 0.2 | 1.5 | 5.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 156.9 ± 0.9 | 155.7 | 166.5 | 7.82 ± 0.86 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 20.1 ± 2.2 | 16.5 | 40.4 | 1.00 |

