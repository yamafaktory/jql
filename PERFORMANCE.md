| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 26.7 ± 0.6 | 25.5 | 31.7 | 14.72 ± 5.65 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 1.8 ± 0.7 | 1.5 | 17.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 26.5 ± 0.5 | 25.4 | 28.1 | 15.45 ± 2.27 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 1.7 ± 0.3 | 1.5 | 5.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 26.6 ± 0.5 | 25.5 | 29.7 | 15.80 ± 2.77 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 1.7 ± 0.3 | 1.5 | 8.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 157.1 ± 1.1 | 155.9 | 175.9 | 7.81 ± 0.91 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 20.1 ± 2.3 | 16.4 | 47.4 | 1.00 |

