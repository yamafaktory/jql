| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 31.8 ± 2.7 | 23.0 | 45.1 | 11.26 ± 5.76 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.8 ± 1.4 | 1.9 | 28.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 32.1 ± 2.5 | 23.7 | 43.8 | 12.19 ± 3.54 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.6 ± 0.7 | 1.8 | 11.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 32.8 ± 2.2 | 26.5 | 47.3 | 11.96 ± 2.72 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.7 ± 0.6 | 2.0 | 9.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 188.3 ± 8.5 | 148.7 | 230.9 | 1.20 ± 0.09 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 157.1 ± 9.5 | 120.7 | 192.7 | 1.00 |

