| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 39.0 ± 2.0 | 35.0 | 53.4 | 16.68 ± 4.10 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.3 ± 0.6 | 1.7 | 9.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 38.3 ± 2.2 | 34.3 | 56.8 | 17.32 ± 5.31 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.2 ± 0.7 | 1.5 | 9.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 38.9 ± 2.1 | 35.0 | 56.2 | 17.63 ± 4.92 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.2 ± 0.6 | 1.5 | 14.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 193.9 ± 7.1 | 177.0 | 235.4 | 7.35 ± 0.92 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 26.4 ± 3.2 | 21.3 | 48.5 | 1.00 |

