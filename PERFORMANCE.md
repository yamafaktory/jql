| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 39.7 ± 2.0 | 36.2 | 54.3 | 17.88 ± 8.11 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.2 ± 1.0 | 1.5 | 24.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 39.9 ± 2.0 | 36.4 | 50.5 | 17.24 ± 4.10 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.3 ± 0.5 | 1.7 | 7.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 39.3 ± 1.7 | 36.5 | 50.6 | 17.14 ± 6.31 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.3 ± 0.8 | 1.6 | 12.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 184.3 ± 6.1 | 173.6 | 219.6 | 1.39 ± 0.07 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 132.2 ± 5.2 | 122.3 | 163.1 | 1.00 |

