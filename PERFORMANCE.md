| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 26.8 ± 0.6 | 25.5 | 29.9 | 14.97 ± 1.87 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 1.8 ± 0.2 | 1.6 | 4.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 26.3 ± 0.5 | 25.1 | 30.8 | 15.25 ± 1.84 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 1.7 ± 0.2 | 1.5 | 2.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 26.5 ± 0.5 | 25.4 | 30.8 | 14.47 ± 1.86 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 1.8 ± 0.2 | 1.6 | 5.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 152.6 ± 1.3 | 150.8 | 179.7 | 7.31 ± 0.80 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 20.9 ± 2.3 | 17.6 | 44.7 | 1.00 |

