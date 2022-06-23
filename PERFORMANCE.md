| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 39.3 ± 1.8 | 32.4 | 53.3 | 16.59 ± 3.57 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.4 ± 0.5 | 1.7 | 8.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 38.9 ± 1.6 | 34.1 | 49.3 | 17.14 ± 8.03 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.3 ± 1.1 | 1.7 | 25.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 33.9 ± 2.8 | 28.7 | 45.2 | 14.10 ± 2.38 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.4 ± 0.4 | 1.6 | 6.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 196.0 ± 7.0 | 165.9 | 223.3 | 1.33 ± 0.07 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 147.0 ± 5.5 | 133.3 | 201.8 | 1.00 |

