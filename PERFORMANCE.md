| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 39.6 ± 2.7 | 34.8 | 59.8 | 19.61 ± 8.46 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.0 ± 0.9 | 1.4 | 16.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 39.9 ± 3.1 | 34.7 | 63.2 | 19.28 ± 5.45 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.1 ± 0.6 | 1.4 | 9.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 39.8 ± 2.8 | 35.5 | 58.9 | 19.72 ± 11.38 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.0 ± 1.2 | 1.2 | 18.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 175.9 ± 10.7 | 159.5 | 227.7 | 1.31 ± 0.12 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 134.2 ± 8.6 | 120.1 | 212.2 | 1.00 |

