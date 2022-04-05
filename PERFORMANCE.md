| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 42.1 ± 2.6 | 36.6 | 53.7 | 17.16 ± 8.64 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.5 ± 1.2 | 1.5 | 28.0 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 41.7 ± 2.7 | 35.9 | 51.9 | 20.72 ± 7.50 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.0 ± 0.7 | 1.1 | 8.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 41.3 ± 2.8 | 36.4 | 58.4 | 20.83 ± 7.44 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.0 ± 0.7 | 1.2 | 9.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 185.4 ± 9.3 | 166.6 | 230.3 | 1.33 ± 0.10 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 139.5 ± 8.2 | 124.7 | 194.2 | 1.00 |

