| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 45.2 ± 2.5 | 40.3 | 60.8 | 17.71 ± 9.00 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.6 ± 1.3 | 1.6 | 19.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 47.5 ± 3.5 | 42.2 | 72.3 | 19.21 ± 7.53 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.5 ± 1.0 | 1.6 | 12.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 46.4 ± 3.5 | 40.3 | 68.8 | 16.21 ± 7.62 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.9 ± 1.3 | 1.7 | 21.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 204.9 ± 8.4 | 185.8 | 246.9 | 1.31 ± 0.09 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 155.9 ± 8.2 | 139.5 | 196.0 | 1.00 |

