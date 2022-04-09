| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 40.1 ± 1.8 | 37.4 | 57.6 | 15.75 ± 6.43 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.5 ± 1.0 | 1.8 | 23.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 39.5 ± 1.8 | 36.2 | 52.9 | 17.74 ± 3.35 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.2 ± 0.4 | 1.7 | 6.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 39.6 ± 1.8 | 35.9 | 53.8 | 15.53 ± 4.38 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.6 ± 0.7 | 1.9 | 15.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 199.7 ± 4.4 | 190.4 | 219.7 | 1.31 ± 0.04 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 152.3 ± 3.7 | 145.0 | 172.3 | 1.00 |

