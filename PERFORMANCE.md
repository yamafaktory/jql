| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 46.0 ± 2.1 | 43.0 | 60.6 | 17.67 ± 6.75 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.6 ± 1.0 | 1.8 | 24.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 45.8 ± 1.7 | 43.2 | 58.5 | 17.80 ± 5.00 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.6 ± 0.7 | 1.9 | 15.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 46.7 ± 2.5 | 40.9 | 70.0 | 18.89 ± 5.95 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.5 ± 0.8 | 1.7 | 13.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 209.7 ± 7.2 | 198.6 | 250.8 | 1.28 ± 0.07 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 163.9 ± 7.1 | 153.4 | 218.0 | 1.00 |

